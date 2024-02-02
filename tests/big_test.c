#include "znes_cpu.h"
#include "znes_memorybus.h"

ZNES_Result znes_cpu_new(ZNES_CPU *cpu)
{
    ZNES_Result result = ZNES_RESULT_OK;
    
    znes_catch(znes_memorybus_new(&cpu->memory), result, finish);
    
    cpu->PC = 0x0;
    cpu->SP = 0x0;
    cpu->A = 0x0;
    cpu->P = 0x0;
    cpu->X = 0x0;
    cpu->Y = 0x0;
    cpu->flags = 0x0;

finish:;
    return result;
}

ZNES_Result znes_cpu_free(ZNES_CPU *cpu)
{
    ZNES_Result result = ZNES_RESULT_OK;

    znes_catch(znes_memorybus_free(&cpu->memory), result, finish)

finish:
    return result;
}

void znes_cpu_set_flag(ZNES_CPU *cpu, ZNES_Flag flag, uint8_t value)
{
    if (value) {
        cpu->flags |= (0x80 >> flag);
    } else {
        cpu->flags &= ~(0x80 >> flag);
    }
}

uint8_t znes_cpu_get_flag(const ZNES_CPU *cpu, ZNES_Flag flag)
{
    return (cpu->flags & (0x80 >> flag)) != 0;
}

ZNES_Result znes_cpu_immediate8(ZNES_CPU *cpu, uint8_t *byte)
{
    ZNES_Result result = ZNES_RESULT_OK;

    znes_catch(znes_memorybus_read8(&cpu->memory, cpu->PC, byte), result, finish);

    cpu->PC++;

finish:;
    return result;
}

ZNES_Result znes_cpu_process_addr(ZNES_CPU *cpu, ZNES_ByteRef *ref, ZNES_AddressingMode mode)
{
    ZNES_Result result = ZNES_RESULT_OK;

    switch (mode) {

    case ZNES_ADDRESSING_ACCUMULATOR: {
        ref->addressing = ZNES_BYTEREF_REGISTER;
        ref->reg.ref = &cpu->A;
        break;
    }

    case ZNES_ADDRESSING_IMMEDIATE: {
        ref->addressing = ZNES_BYTEREF_LITERAL;
        znes_catch(znes_cpu_immediate8(cpu, &ref->literal), result, finish);
        break;
    }

    case ZNES_ADDRESSING_ABSOLUTE: {
        uint8_t least;
        uint8_t most;
        znes_catch(znes_cpu_immediate8(cpu, &least), result, finish);
        znes_catch(znes_cpu_immediate8(cpu, &most), result, finish);
        uint16_t addr = (uint16_t)least | (((uint16_t)most) << 8);
        znes_catch(znes_memorybus_read8ref(&cpu->memory, addr, ref), result, finish);
        break;
    }

    case ZNES_ADDRESSING_ZERO_PAGE: {
        uint8_t imm;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        znes_catch(znes_memorybus_read8ref(&cpu->memory, (uint16_t)imm, ref), result, finish);
        break;
    }

    case ZNES_ADDRESSING_RELATIVE: {
        uint16_t old_pc = cpu->PC;
        uint8_t imm;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        int8_t sign = (int8_t)imm;
        znes_catch(znes_memorybus_read8ref(&cpu->memory, old_pc + sign, ref), result, finish);
        break;
    }

    case ZNES_ADDRESSING_ZERO_X: {
        uint8_t imm;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        znes_catch(znes_memorybus_read8ref(&cpu->memory, (imm + cpu->X) % 256, ref), result, finish);
        break;
    }

    case ZNES_ADDRESSING_ZERO_Y: {
        uint8_t imm;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        znes_catch(znes_memorybus_read8ref(&cpu->memory, (imm + cpu->Y) % 256, ref), result, finish);
        break; 
    }

    case ZNES_ADDRESSING_ABS_X: {
        uint8_t imm;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        znes_catch(znes_memorybus_read8ref(&cpu->memory, imm + cpu->X, ref), result, finish);
        break; 
    }

    case ZNES_ADDRESSING_ABS_Y: {
        uint8_t imm;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        znes_catch(znes_memorybus_read8ref(&cpu->memory, imm + cpu->Y, ref), result, finish);
        break; 
    }

    case ZNES_ADDRESSING_INDIRECT_X: {
        uint8_t imm;
        uint8_t byte_1;
        uint8_t byte_2;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        znes_catch(znes_memorybus_read8(&cpu->memory, (imm + cpu->X) % 256, &byte_1), result, finish);
        znes_catch(znes_memorybus_read8(&cpu->memory, (imm + cpu->X + 1) % 256, &byte_2), result, finish);
        uint16_t addr = (byte_1 + byte_2) * 256;
        znes_catch(znes_memorybus_read8ref(&cpu->memory, addr, ref), result, finish);
        break;
    }

    case ZNES_ADDRESSING_INDIRECT_Y: {
        uint8_t imm;
        uint8_t byte_1;
        uint8_t byte_2;
        znes_catch(znes_cpu_immediate8(cpu, &imm), result, finish);
        znes_catch(znes_memorybus_read8(&cpu->memory, imm, &byte_1), result, finish);
        znes_catch(znes_memorybus_read8(&cpu->memory, (imm + 1) % 256, &byte_2), result, finish);
        uint16_t addr = ((uint16_t)byte_1 + (uint16_t)byte_2) * 256 + cpu->Y;
        znes_catch(znes_memorybus_read8ref(&cpu->memory, addr, ref), result, finish);
        break;
    }

    }

finish:;
    return result;
}

ZNES_Result znes_cpu_exec_next(ZNES_CPU *cpu)
{
    ZNES_Result result = ZNES_RESULT_OK;
    uint8_t op;
    znes_catch(znes_memorybus_read8(&cpu->memory, cpu->PC, &op), result, finish);
    cpu->PC++;

    switch (op)
    {
    }

finish:;
    return result;
}

ZNES_Result znes_cpu_instr_adc(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{
    
}

ZNES_Result znes_cpu_instr_and(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_asl(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_bcc(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_bcs(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_beq(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_bit(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_bmi(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_bne(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_bpl(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_brk(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_bvc(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_bvs(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_flag_clear(ZNES_CPU *cpu, const ZNES_Flag flag)
{
    znes_cpu_set_flag(cpu, flag, 0);
    return ZNES_RESULT_OK;
}

ZNES_Result znes_cpu_instr_cmp(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_cpx(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_cpy(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_dec(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{
    ZNES_Result result = ZNES_RESULT_OK;
    ZNES_ByteRef ref;
    znes_catch(znes_cpu_process_addr(cpu, &ref, mode), result, finish);

    uint8_t value;
    znes_catch(znes_byteref_read(&ref, &value), result, finish);
    value--;
    znes_catch(znes_byteref_write(&ref, value));

    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (value & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, value == 0);

finish:;
    return result;
}

ZNES_Result znes_cpu_instr_dex(ZNES_CPU *cpu)
{
    cpu->X--;
    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (cpu->X & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, cpu->X == 0);
    return ZNES_RESULT_OK;
}

ZNES_Result znes_cpu_instr_dey(ZNES_CPU *cpu)
{
    cpu->Y--;
    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (cpu->Y & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, cpu->Y == 0);
    return ZNES_RESULT_OK;
}

ZNES_Result znes_cpu_instr_eor(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_inc(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{
    ZNES_Result result = ZNES_RESULT_OK;
    ZNES_ByteRef ref;
    znes_catch(znes_cpu_process_addr(cpu, &ref, mode), result, finish);

    uint8_t value;
    znes_catch(znes_byteref_read(&ref, &value), result, finish);
    value++;
    znes_catch(znes_byteref_write(&ref, value));

    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (value & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, value == 0);

finish:;
    return result;
}

ZNES_Result znes_cpu_instr_inx(ZNES_CPU *cpu)
{
    cpu->X++;
    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (cpu->X & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, cpu->X == 0);
    return ZNES_RESULT_OK;
}

ZNES_Result znes_cpu_instr_iny(ZNES_CPU *cpu)
{
    cpu->Y++;
    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (cpu->Y & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, cpu->Y == 0);
    return ZNES_RESULT_OK;
}

ZNES_Result znes_cpu_instr_jmp(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_jsr(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_lda(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{
    ZNES_Result result = ZNES_RESULT_OK;
    ZNES_ByteRef ref;
    znes_catch(znes_cpu_process_addr(cpu, &ref, mode), result, finish);

    uint8_t value;
    znes_catch(znes_byteref_read(&ref, &value), result, finish);
    cpu->A = value;

    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (value & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, value == 0);

finish:;
    return result;
}

ZNES_Result znes_cpu_instr_ldx(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{
    ZNES_Result result = ZNES_RESULT_OK;
    ZNES_ByteRef ref;
    znes_catch(znes_cpu_process_addr(cpu, &ref, mode), result, finish);

    uint8_t value;
    znes_catch(znes_byteref_read(&ref, &value), result, finish);
    cpu->X = value;

    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (value & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, value == 0);

finish:;
    return result;

}

ZNES_Result znes_cpu_instr_ldy(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{
    ZNES_Result result = ZNES_RESULT_OK;
    ZNES_ByteRef ref;
    znes_catch(znes_cpu_process_addr(cpu, &ref, mode), result, finish);

    uint8_t value;
    znes_catch(znes_byteref_read(&ref, &value), result, finish);
    cpu->Y = value;

    znes_cpu_set_flag(cpu, ZNES_FLAG_NEGATIVE, (value & 0x80));
    znes_cpu_set_flag(cpu, ZNES_FLAG_ZERO, value == 0);

finish:;
    return result;
}

ZNES_Result znes_cpu_instr_lsr(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{
    
}

ZNES_Result znes_cpu_instr_ora(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_pha(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_php(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_pla(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_plp(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_rol(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_ror(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_rti(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_rts(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_sbc(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_flag_set(ZNES_CPU *cpu, const ZNES_Flag flag)
{
    znes_cpu_set_flag(cpu, flag, 1);
    return ZNES_RESULT_OK;
}

ZNES_Result znes_cpu_instr_sta(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_stx(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_sty(ZNES_CPU *cpu, const ZNES_AddressingMode mode)
{

}

ZNES_Result znes_cpu_instr_tax(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_tay(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_tsx(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_txa(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_txs(ZNES_CPU *cpu)
{

}

ZNES_Result znes_cpu_instr_tya(ZNES_CPU *cpu)
{

}
